import { useContext, useState } from "react";
import { useClient } from "@hooks/useClient.ts";
import { type AuthContext, authContext } from "@contexts/AuthContext.ts";

import type { SignUpRequest } from "@bindings/SignUpRequest.ts";
import type { SignInResponse } from "@bindings/SignInResponse.ts";
import type { SignUpResponse } from "@bindings/SignUpResponse.ts";
import type { SignInRequest } from "@bindings/SignInRequest.ts";
import type { OneTimeRequest } from "@bindings/OneTimeRequest.ts";
import type { OneTimeResponse } from "@bindings/OneTimeResponse.ts";

export const useAuth = (): AuthContext | null => {
	// throw new Error("AuthContext was not provided");
	return useContext(authContext);
};

const signUp = async (data: SignUpRequest): Promise<SignUpResponse> => {
	const [client] = useClient();
	const response = await client.post<SignUpResponse>("/signup", data);
	return response.data;
};

const signIn = async (data: SignInRequest): Promise<SignInResponse> => {
	const [client] = useClient();
	const response = await client.post<SignInResponse>("/signin", data);
	return response.data;
};

const oneTime = async (data: OneTimeRequest): Promise<OneTimeResponse> => {
	const [client] = useClient();
	const response = await client.post<OneTimeResponse>("/onetime", data);
	return response.data;
};

export const useProvideAuth = () => {
	const [user, setUser] = useState(null);

	const signUp = () => {};

	const signIn = () => {};

	return {};
};
