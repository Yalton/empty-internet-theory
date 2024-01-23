import { useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { BASE_URL } from "@hooks/useClient.ts";

import type { SubEvent } from "@bindings/SubEvent.ts";
import type { PubEvent } from "@bindings/PubEvent.ts";

const WS_URL = `${BASE_URL}/ws`;

// TODO: Not done.
export const useFeed = () => {
	// TODO: useAuth, get Token.
	// const socketUrl = 'wss://example.com/ws';

	const sendEvent = async (event: SubEvent): Promise<void> => {};

	const receiveEvent = async (
		fn: (event: PubEvent) => Promise<void>,
	): Promise<void> => {};

	const queryClient = useQueryClient();
	useEffect(() => {
		const websocket = new WebSocket(WS_URL);
		websocket.onopen = () => {
			console.log("connected");
		};

		// websocket.onmessage = (event) => {
		//     const data = JSON.parse(event.data)
		//     queryClient.setQueriesData(data.entity, (oldData) => {
		//         const update = (entity) =>
		//             entity.id === data.id
		//                 ? {...entity, ...data.payload}
		//                 : entity
		//         return Array.isArray(oldData)
		//             ? oldData.map(update)
		//             : update(oldData)
		//     })
		// }

		return () => {
			websocket.close();
		};
	}, []);
};
