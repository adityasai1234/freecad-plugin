'use client';

import { useChat } from '@ai-sdk/react';
import { DefaultChatTransport } from 'ai';
import { useEffect, useRef, useState } from 'react';
import type { ProviderName } from '@/lib/providers';

interface Props {
  provider: ProviderName;
  model: string;
}

export function Chat({ provider, model }: Props) {
  const [input, setInput] = useState('');
  const bottomRef = useRef<HTMLDivElement>(null);

  const { messages, sendMessage, status } = useChat({
    transport: new DefaultChatTransport({
      api: '/api/chat',
      body: { provider, model },
    }),
  });

  const isLoading = status === 'streaming' || status === 'submitted';

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  return (
    <div className="flex flex-col h-full">
      <div className="flex-1 overflow-y-auto px-4 py-4 space-y-4">
        {messages.length === 0 && (
          <div className="text-center text-gray-500 mt-16 text-sm">
            Ask me to create boxes, cylinders, boolean operations, or export STL/STEP files.
          </div>
        )}

        {messages.map(m => (
          <div key={m.id} className={`flex ${m.role === 'user' ? 'justify-end' : 'justify-start'}`}>
            <div className={`max-w-[80%] rounded-lg px-4 py-2 text-sm ${
              m.role === 'user' ? 'bg-blue-600 text-white' : 'bg-gray-800 text-gray-100'
            }`}>
              {m.parts.map((part, i) => {
                if (part.type === 'text') {
                  return <p key={i} className="whitespace-pre-wrap">{part.text}</p>;
                }
                if (part.type.startsWith('tool-')) {
                  const toolName = part.type.slice(5);
                  const isResult = 'result' in part;
                  return (
                    <div key={i} className="mt-2 text-xs font-mono bg-gray-900 rounded p-2">
                      <div className="text-yellow-400 mb-1">
                        {isResult ? '✓' : '⏳'} {toolName}
                      </div>
                      {'input' in part && (
                        <div className="text-gray-400">{JSON.stringify(part.input, null, 2)}</div>
                      )}
                      {isResult && (
                        <div className="mt-1 text-green-400">
                          {typeof part.result === 'string' ? part.result : JSON.stringify(part.result, null, 2)}
                        </div>
                      )}
                    </div>
                  );
                }
                return null;
              })}
            </div>
          </div>
        ))}

        <div ref={bottomRef} />
      </div>

      <form
        onSubmit={(e: React.FormEvent<HTMLFormElement>) => {
          e.preventDefault();
          if (!input.trim() || isLoading) return;
          sendMessage({ text: input });
          setInput('');
        }}
        className="border-t border-gray-800 p-4 flex gap-2"
      >
        <input
          value={input}
          onChange={e => setInput(e.target.value)}
          placeholder="Create a box 100x50x25, export as STL..."
          disabled={isLoading}
          className="flex-1 bg-gray-800 border border-gray-700 rounded-lg px-4 py-2 text-sm text-gray-100 placeholder-gray-500 focus:outline-none focus:ring-1 focus:ring-blue-500 disabled:opacity-50"
        />
        <button
          type="submit"
          disabled={isLoading || !input.trim()}
          className="bg-blue-600 hover:bg-blue-500 disabled:opacity-50 disabled:cursor-not-allowed text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors"
        >
          Send
        </button>
      </form>
    </div>
  );
}
