import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Outlet } from "@tanstack/react-router";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";

import LoginPage from "./loginpage/LoginPage";
import Lobby from "./lobby/Lobby";
import FeedPage from "./feed/Feed";
import ProfilePage from "./profilepage/ProfilePage";
import PostGame from "./postgame/PostGame";

import "./App.css";

const Layout = () => {
	return (
		<>
			<Outlet />
		</>
	);
};

const App = () => {
	return (
		<Router>
			<Routes>
				<Route path="/" element={<LoginPage />} />
				<Route path="/lobby" element={<Lobby />} />
				<Route path="/pregame" element={<PostGame />} />
				<Route path="/timeline" element={<FeedPage />} />
				<Route path="/profile" element={<ProfilePage />} />
				<Route path="/postgame" element={<PostGame />} />
			</Routes>
		</Router>
	);
};

const AppWithProviders = () => {
	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				staleTime: Infinity,
			},
		},
	});

	return (
		<QueryClientProvider client={queryClient}>
			<Layout />
			<App />
		</QueryClientProvider>
	);
};

export default AppWithProviders;
