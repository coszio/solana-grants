/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    domains: ["api.lorem.space"],
  },
  output: "standalone",
};

module.exports = nextConfig
