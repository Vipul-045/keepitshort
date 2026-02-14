import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  async rewrites() {
  return [
    {
      source: '/:shortcode',
      destination: 'https://keepitshort.onrender.com/:shortcode',
    },
  ];
}
};



export default nextConfig;
