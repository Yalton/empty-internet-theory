import { internalIpV4 } from "internal-ip";
import { dirname, join } from "node:path";
import react from "@vitejs/plugin-react-swc";
import { defineConfig, type HmrOptions } from "vite";

const platform = process.env.TAURI_ENV_PLATFORM as string;
const mobile = !!/android|ios/.exec(platform);
// TODO: const mobile = /android|ios/.test(platform);

const viteConfig = new URL(import.meta.url).pathname;
const currentDir = dirname(viteConfig);

const createHmrOptions = async (): Promise<HmrOptions> => {
	return {
		protocol: "ws",
		host: await internalIpV4(),
		port: 1421,
	};
};

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [react()],
	publicDir: join(currentDir, "./public"),
	resolve: {
		alias: {
			"@assets": join(currentDir, "./assets"),
			"@components": join(currentDir, "./components"),
			"@contexts": join(currentDir, "./contexts"),
			"@hooks": join(currentDir, "./hooks"),
			"@providers": join(currentDir, "./providers"),
			"@reducers": join(currentDir, "./reducers"),
			"@styles": join(currentDir, "./styles"),
		},
	},
	build: {
		outDir: "build",
		assetsDir: ".",
	},

	// Prevent Vite from obscuring Rust errors.
	clearScreen: false,
	// Tauri expects a fixed port, fail if that port is not available.
	server: {
		port: 1420,
		strictPort: true,
		host: mobile ? "0.0.0.0" : false,
		hmr: mobile ? await createHmrOptions() : undefined,
		watch: {
			ignored: ["**/native/**"],
		},
	},
}));
