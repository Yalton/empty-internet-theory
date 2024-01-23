import { useClient } from "@hooks/useClient.ts";
import { useQuery } from "@tanstack/react-query";
import type { Status } from "@bindings/Status.ts";

const getStatus = async (): Promise<Status> => {
	const [client] = useClient();
	const response = await client.get<Status>("/status");
	return response.data;
};

export const useStatus = () => {
	return useQuery({
		queryKey: ["status"],
		queryFn: getStatus,
	});
};
