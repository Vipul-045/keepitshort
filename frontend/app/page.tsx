'use client';

import { useState } from 'react';
import { short_url } from './api';

export default function Home() {
  const [longUrl, setLongUrl] = useState('');
  const [shortUrl, setShortUrl] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleShorten = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!longUrl) return;

    setIsLoading(true);
    
    const data = await short_url(longUrl)
    setShortUrl(data)
    setIsLoading(false);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(shortUrl);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-black selection:bg-blue-500 selection:text-white relative overflow-hidden">
    <>  
    <h2 className='mb-5 text-2xl font-bold items-start justify-start'>Keep It Short</h2>
      
      {/* Content Container */}
      <div className="relative z-10 w-full max-w-lg px-6">
        
        {/* Input Section */}
        <form onSubmit={handleShorten} className="flex flex-col gap-4">
          <div className="relative group">
            <div className="absolute -inset-1  from-blue-600 to-purple-600 rounded-lg blur opacity-25 group-hover:opacity-75 transition duration-1000 group-hover:duration-200"></div>
            <input
              type="url"
              placeholder="Enter long URL here..."
              value={longUrl}
              onChange={(e) => setLongUrl(e.target.value)}
              required
              className="relative w-full bg-white/5 border border-white/10 text-white placeholder-gray-500 rounded-lg px-6 py-4 focus:outline-none focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500 transition-all shadow-[0_0_0_1px_rgba(255,255,255,0.1)]"
            />
          </div>

          <button
            type="button"
            onClick={handleShorten}
            disabled={isLoading}
            className="mx-auto flex flex-col items-center justify-center w-1/2 text-white cursor-pointer font-bold py-3 rounded-lg shadow-[0_0_30px_rgba(57,99,235,1)] hover:shadow-[0_0_20px_rgba(37,99,235,0.5)] transform transition-all active:scale-[0.98] disabled:opacity-70 disabled:cursor-not-allowed"
          >
            {isLoading ? 'Shortening...' : 'Short it'}
          </button>
        </form>

        {/* Result Section */}
        {shortUrl && (
          <div className="mt-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
            <div className="p-1 rounded-xl from-white/10 to-transparent">
              <div className="bg-black/80 backdrop-blur-md border border-white/10 rounded-lg p-4 flex items-center justify-between gap-4">
                
                <span className="text-blue-400 font-mono text-lg truncate selection:bg-white/20">
                  {shortUrl}
                </span>

                <button
                  onClick={handleCopy}
                  className={`flex items-center gap-2 px-4 py-2 rounded-md font-medium transition-all ${
                    copied 
                      ? 'bg-green-500/20 text-green-400 hover:bg-green-500/30' 
                      : 'bg-blue-600 text-white hover:bg-blue-500'
                  }`}
                >
                  {copied ? (
                    <>
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
                      Copied
                    </>
                  ) : (
                    <>
                      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
                      Copy
                    </>
                  )}
                </button>
                
              </div>
            </div>
          </div>
        )}

        {}
      </div>
        </>
      <div className="absolute bottom-6 left-0 right-0 text-center text-white/50 text-sm font-light">
  built by <a href="https://vipul.live" className='underline'>Vipul</a>
</div>
    </main>
  );
}