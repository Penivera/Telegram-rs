import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import Home from './components/Home';
import ApiDocs from './components/ApiDocs';
import Examples from './components/Examples';
import Demos from './components/Demos';
import './styles.css';

function App() {
  const [darkMode, setDarkMode] = useState(false);

  useEffect(() => {
    const prefersDark = window.matchMedia(
      '(prefers-color-scheme: dark)'
    ).matches;
    setDarkMode(prefersDark);
    if (prefersDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, []);

  const toggleTheme = () => {
    setDarkMode(!darkMode);
    if (!darkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  };

  return (
    <Router>
      <div className="flex">
        <aside className="sidebar">
          <h1 className="text-2xl font-bold mb-4">Telegram-rs Docs</h1>
          <nav>
            <Link to="/" className="block mb-2">Home</Link>
            <Link to="/api" className="block mb-2">API Docs</Link>
            <Link to="/examples" className="block mb-2">Examples</Link>
            <Link to="/demos" className="block mb-2">Demos</Link>
          </nav>
          <button
            onClick={toggleTheme}
            className="mt-4 p-2 bg-primary text-white rounded flex items-center justify-center"
          >
            {darkMode ? '🌞 Light Mode' : '🌙 Dark Mode'}
          </button>
        </aside>
        <main className="main-content">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/api" element={<ApiDocs />} />
            <Route path="/examples" element={<Examples />} />
            <Route path="/demos" element={<Demos />} />
          </Routes>
        </main>
      </div>
      <footer className="footer w-full mt-8">
        <p>© 2026 Telegram-rs. All rights reserved. | <a href="https://github.com/Penivera/Telegram-rs" className="text-primary">GitHub</a></p>
      </footer>
    </Router>
  );
}

export default App;
