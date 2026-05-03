import type { LanguageModel } from 'ai';

export type ProviderName = 'openai' | 'gemini' | 'ollama';

export interface ModelOption {
  id: string;
  label: string;
}

export const PROVIDER_MODELS: Record<ProviderName, ModelOption[]> = {
  openai: [
    { id: 'gpt-4o', label: 'GPT-4o' },
    { id: 'gpt-4o-mini', label: 'GPT-4o Mini' },
    { id: 'gpt-4-turbo', label: 'GPT-4 Turbo' },
  ],
  gemini: [
    { id: 'gemini-2.0-flash', label: 'Gemini 2.0 Flash' },
    { id: 'gemini-1.5-pro', label: 'Gemini 1.5 Pro' },
    { id: 'gemini-1.5-flash', label: 'Gemini 1.5 Flash' },
  ],
  ollama: [
    { id: 'llama3.2', label: 'Llama 3.2' },
    { id: 'qwen2.5-coder', label: 'Qwen 2.5 Coder' },
    { id: 'mistral', label: 'Mistral' },
    { id: 'codellama', label: 'Code Llama' },
  ],
};

export function getModel(_provider: ProviderName, _modelId: string): LanguageModel {
  throw new Error('getModel not yet implemented');
}
