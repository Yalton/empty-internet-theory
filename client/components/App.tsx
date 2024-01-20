import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import LoginPage from "./loginpage/LoginPage";
import Lobby from "./lobby/Lobby";
import FeedPage from "./feed/Feed";
// import ProfilePage from "./profilepage/ProfilePage";
import "./App.css";



function App() {
	return (
		<Router>
			<Routes>
				<Route path="/" element={<LoginPage />} />
				<Route path="/lobby" element={<Lobby />} />
				<Route path="/timeline" element={<FeedPage />} />
				{/* <Route path="/profile" element={<ProfilePage />} /> */}
			</Routes>
		</Router>
	);
}

export default App;
