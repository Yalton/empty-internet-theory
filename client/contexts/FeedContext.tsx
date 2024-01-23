import {createContext, type ReactNode} from "react";

export interface FeedContext {
}

export const feedContext = createContext<FeedContext | null>(null);

export interface FeedProviderProps {
    children?: ReactNode;
}

export function FeedProvider({children}: FeedProviderProps) {
    return <feedContext.Provider value={}>{children}</feedContext.Provider>;
}
