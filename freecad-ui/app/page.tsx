'use client';

import { useState } from 'react';
import { Chat } from '@/components/chat';
import { ModelSelector } from '@/components/model-selector';
import { PROVIDER_MODELS, type ProviderName } from '@/lib/providers';

export default function Page() {
  const [provider, setProvider] = useState<ProviderName>('openai');
  const [model, setModel] = useState(PROVIDER_MODELS.openai[0].id);

  return (
    <div className="flex flex-col h-screen">
      {/* Header */}
      <header className="flex items-center justify-between px-4 py-3 border-b border-gray-800">
        <span className="font-semibold text-sm text-gray-100">FreeCAD Chat</span>
        <ModelSelector
          provider={provider}
          model={model}
          onProviderChange={p => {
            setProvider(p);
            setModel(PROVIDER_MODELS[p][0].id);
          }}
          onModelChange={setModel}
        />
      </header>

      {/* Chat — remount on provider/model change to reset history */}
      <div className="flex-1 overflow-hidden">
        <Chat key={`${provider}:${model}`} provider={provider} model={model} />
      </div>
    </div>
  );
}
