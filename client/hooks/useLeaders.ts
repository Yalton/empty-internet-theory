import { useClient } from "@hooks/useClient.ts";
import { useQuery } from "@tanstack/react-query";

import type { LeaderboardResponse } from "@bindings/LeaderboardResponse.ts";
import type { LeaderboardRequest } from "@bindings/LeaderboardRequest.ts";

const getLeaderboards = async (
	request?: LeaderboardRequest,
): Promise<LeaderboardResponse> => {
	const [client] = useClient();
	const response = await client.get<LeaderboardResponse>("/leaderboard", {
		data: request,
	});

	return response.data;
};

export const useLeaders = () => {
	return useQuery({
		queryKey: ["leaderboard"],
		queryFn: () => getLeaderboards,
	});
};
