'use client';

import { PROVIDER_MODELS, type ProviderName } from '@/lib/providers';

interface Props {
  provider: ProviderName;
  model: string;
  onProviderChange: (p: ProviderName) => void;
  onModelChange: (m: string) => void;
}

const PROVIDER_LABELS: Record<ProviderName, string> = {
  openai: 'OpenAI',
  gemini: 'Gemini',
  ollama: 'Ollama (local)',
};

export function ModelSelector({ provider, model, onProviderChange, onModelChange }: Props) {
  const models = PROVIDER_MODELS[provider];

  return (
    <div className="flex items-center gap-2">
      <select
        value={provider}
        onChange={e => {
          const p = e.target.value as ProviderName;
          onProviderChange(p);
          onModelChange(PROVIDER_MODELS[p][0].id);
        }}
        className="bg-gray-800 border border-gray-700 rounded px-2 py-1 text-sm text-gray-200 focus:outline-none focus:ring-1 focus:ring-blue-500"
      >
        {(Object.keys(PROVIDER_LABELS) as ProviderName[]).map(p => (
          <option key={p} value={p}>{PROVIDER_LABELS[p]}</option>
        ))}
      </select>

      <select
        value={model}
        onChange={e => onModelChange(e.target.value)}
        className="bg-gray-800 border border-gray-700 rounded px-2 py-1 text-sm text-gray-200 focus:outline-none focus:ring-1 focus:ring-blue-500"
      >
        {models.map(m => (
          <option key={m.id} value={m.id}>{m.label}</option>
        ))}
      </select>
    </div>
  );
}
