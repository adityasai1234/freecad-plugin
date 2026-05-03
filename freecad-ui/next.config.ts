import type { NextConfig } from 'next';

const config: NextConfig = {
  // Allow the MCP stdio transport to spawn subprocesses
  serverExternalPackages: ['@ai-sdk/mcp'],
};

export default config;
