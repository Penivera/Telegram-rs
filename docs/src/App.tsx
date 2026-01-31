import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import Home from './components/Home';
import ApiDocs from './components/ApiDocs';
import Examples from './components/Examples';
import Demos from './components/Demos';
import './App.css';

function App() {
  return (
    <Router>
      <div className="min-h-screen bg-gray-100">
        <nav className="bg-blue-600 p-4">
          <div className="container mx-auto flex space-x-4">
            <Link to="/" className="text-white hover:text-gray-200">Home</Link>
            <Link to="/api" className="text-white hover:text-gray-200">API Docs</Link>
            <Link to="/examples" className="text-white hover:text-gray-200">Examples</Link>
            <Link to="/demos" className="text-white hover:text-gray-200">Demos</Link>
          </div>
        </nav>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/api" element={<ApiDocs />} />
          <Route path="/examples" element={<Examples />} />
          <Route path="/demos" element={<Demos />} />
        </Routes>
      </div>
    </Router>
  );
}

export default App;
