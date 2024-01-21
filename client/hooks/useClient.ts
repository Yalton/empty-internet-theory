import axios from "axios";

const apiClient = axios.create({
	// baseURL: 'https://your-api-url.com',
	baseURL: "http://127.0.0.1:3000",
	decompress: true,
	headers: {
		"Content-Type": "application/json",
	},
});

export const useClient = (): [typeof apiClient] => {
	return [apiClient];
};
