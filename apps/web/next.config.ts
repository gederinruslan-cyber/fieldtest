import type { NextConfig } from "next";

const config: NextConfig = {
  poweredByHeader: false,
  transpilePackages: ["@fieldtest/engine-wasm"],
};

export default config;
