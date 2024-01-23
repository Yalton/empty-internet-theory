import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import LoginPage from "./loginpage/LoginPage";
import Lobby from "./lobby/Lobby";
import FeedPage from "./feed/Feed";
import ProfilePage from "./profilepage/ProfilePage";
import PostGame from "./postgame/PostGame";

import "./App.css";

function App() {
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
}

export default App;
