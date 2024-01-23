import { createContext, type ReactNode } from "react";
import { useProvideAuth } from "@hooks/useAuth.ts";

export interface AuthContext {
	username: string;
}

export const authContext = createContext<AuthContext | null>(null);

export interface AuthProviderProps {
	children?: ReactNode;
}

export function AuthProvider({ children }: AuthProviderProps) {
	const auth = useProvideAuth();
	return <authContext.Provider value={auth}>{children}</authContext.Provider>;
}
