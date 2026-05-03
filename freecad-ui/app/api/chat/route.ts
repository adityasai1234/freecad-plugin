import { streamText, convertToModelMessages, type UIMessage } from 'ai';
import { getModel, type ProviderName } from '@/lib/providers';

export async function POST(req: Request) {
  const { messages, provider, model: modelId } = await req.json() as {
    messages: UIMessage[];
    provider: ProviderName;
    model: string;
  };

  const model = getModel(provider ?? 'openai', modelId ?? 'gpt-4o-mini');
  const modelMessages = await convertToModelMessages(messages);

  const result = streamText({
    model,
    messages: modelMessages,
    system:
      'You are a FreeCAD assistant. Use the available tools to create, manipulate, and export 3D objects. ' +
      'Always confirm successful operations and report object IDs so the user can reference them.',
  });

  return result.toUIMessageStreamResponse();
}
