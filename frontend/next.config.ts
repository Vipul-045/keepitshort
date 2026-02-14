import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  async rewrites() {
  return [
    {
      source: '/:shortcode',
      destination: 'https://keepitshort-46ze.onrender.com/:shortcode',
    },
  ];
}
};



export default nextConfig;
