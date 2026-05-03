import { streamText, convertToModelMessages, stepCountIs, type UIMessage } from 'ai';
import { createMCPClient } from '@ai-sdk/mcp';
import { Experimental_StdioMCPTransport } from '@ai-sdk/mcp/mcp-stdio';
import { getModel, type ProviderName } from '@/lib/providers';

const FREECAD_MCP_COMMAND =
  process.env.FREECAD_MCP_COMMAND ??
  '../freecad-mcp/target/debug/freecad-mcp';

export async function POST(req: Request) {
  const { messages, provider, model: modelId } = await req.json() as {
    messages: UIMessage[];
    provider: ProviderName;
    model: string;
  };

  const model = getModel(provider ?? 'openai', modelId ?? 'gpt-4o-mini');

  let mcpClient: Awaited<ReturnType<typeof createMCPClient>> | undefined;

  try {
    mcpClient = await createMCPClient({
      transport: new Experimental_StdioMCPTransport({
        command: FREECAD_MCP_COMMAND,
        args: [],
      }),
    });

    const tools = await mcpClient.tools();
    const modelMessages = await convertToModelMessages(messages);

    const result = streamText({
      model,
      messages: modelMessages,
      tools,
      stopWhen: stepCountIs(15),
      system:
        'You are a FreeCAD assistant. Use the available tools to create, manipulate, and export 3D objects. ' +
        'Always confirm successful operations and report object IDs so the user can reference them.',
      onFinish: async () => { await mcpClient?.close(); },
      onError: async () => { await mcpClient?.close(); },
    });

    return result.toUIMessageStreamResponse();
  } catch (err) {
    await mcpClient?.close();
    console.error('[chat/route]', err);
    return new Response(
      JSON.stringify({ error: err instanceof Error ? err.message : 'Unknown error' }),
      { status: 500, headers: { 'Content-Type': 'application/json' } },
    );
  }
}
