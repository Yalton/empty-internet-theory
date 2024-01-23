import axios from "axios";

export const BASE_URL = "http://127.0.0.1:3000";

const apiClient = axios.create({
	baseURL: BASE_URL,
	decompress: true,
	headers: {
		"Content-Type": "application/json",
	},
});

export const useClient = (): [typeof apiClient] => {
	return [apiClient];
};
