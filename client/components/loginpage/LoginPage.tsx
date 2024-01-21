import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useStatus } from "@hooks/useStatus.ts"; // Import useNavigate

const LoginPage = () => {
	const { data, status } = useStatus();
	const [username, setUsername] = useState("");
	const [password, setPassword] = useState("");
	const navigate = useNavigate(); // Initialize the navigate function

	const handleLogin = () => {
		alert("Login attempt with: " + username + " " + password);
		navigate("/lobby");
	};

	return (
		<div className="app">
			<span className={"text-pink-500 font-bold"}>
				status: {status}, data: [{data && JSON.stringify(data)}]
			</span>

			<div className="login-container">
				<div className="login-form">
					<input
						type="text"
						placeholder="Username"
						value={username}
						onChange={(e) => setUsername(e.target.value)}
						className="login-input"
					/>
					<input
						type="password"
						placeholder="Password"
						value={password}
						onChange={(e) => setPassword(e.target.value)}
						className="login-input"
					/>
					<button onClick={handleLogin} className="login-button">
						Login
					</button>
				</div>
			</div>
		</div>
	);
};

export default LoginPage;
