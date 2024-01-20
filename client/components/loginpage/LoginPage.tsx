import React, { useState } from "react";
import BottomNav from "../common/BottomNav";
import "./App.css";

const LoginPage = () => {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");

  const handleLogin = () => {
    alert("Login attempt with: " + username + " " + password);
  };

  return (
    <div className="app">
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
      <BottomNav />
    </div>
  );
};

export default LoginPage;
