import React from 'react';

const Home: React.FC = () => {
  return (
    <div className="p-8 max-w-4xl mx-auto">
      <h1 className="text-4xl font-bold mb-6">Telegram-rs Documentation</h1>
      <p className="text-lg mb-4">
        A comprehensive Rust library for building Telegram bots and Mini Apps with optional wallet integration.
      </p>
      <h2 className="text-2xl font-semibold mb-4">Features</h2>
      <ul className="list-disc list-inside mb-6">
        <li>Telegram Bot API: Asynchronous client for sending/receiving messages, handling updates via polling or webhooks</li>
        <li>Telegram Mini Apps (feature-gated): Utilities for creating web apps within Telegram with initData validation and WASM bridge support</li>
        <li>Wallet Integration (feature-gated): TON Connect v2 support for blockchain-enabled bots and Mini Apps</li>
      </ul>
      <h2 className="text-2xl font-semibold mb-4">Installation</h2>
      <pre className="bg-gray-200 p-4 rounded mb-6">
        <code>
{`[dependencies]
telegram-rs = "0.1"

# With Mini App support
telegram-rs = { version = "0.1", features = ["mini-app"] }

# With Wallet integration
telegram-rs = { version = "0.1", features = ["wallet"] }

# With all features
telegram-rs = { version = "0.1", features = ["mini-app", "wallet"] }`}
        </code>
      </pre>
      <p>Check out the <a href="/api" className="text-blue-600">API Docs</a>, <a href="/examples" className="text-blue-600">Examples</a>, and <a href="/demos" className="text-blue-600">Demos</a>.</p>
    </div>
  );
};

export default Home;