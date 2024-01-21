import { useClient } from "@hooks/useClient.ts";
import { useQuery } from "@tanstack/react-query";

import type { StatusRequest } from "@bindings/StatusRequest.ts";
import type { StatusResponse } from "@bindings/StatusResponse.ts";

const getStatus = async (req: StatusRequest): Promise<StatusResponse> => {
	const [client] = useClient();
	const response = await client.get<StatusResponse>("/status", {
		responseType: "json",
		data: req,
	});

	console.log(response);
	return response.data;
};

export const useStatus = (verbose = false) => {
	return useQuery({
		queryKey: ["status"],
		queryFn: () => getStatus({ verbose }),
	});
};
