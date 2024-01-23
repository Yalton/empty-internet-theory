import React from "react";
import { useNavigate } from "react-router-dom";
import "../App.css";

const BottomNav = () => {
  const navigate = useNavigate();

  return (
    <>
      <hr className="post-divider" />
      <div className="bottom-nav">
        <div className="nav-item" onClick={() => navigate("/timeline")}>
          <i className="icon home-icon"></i>
          <span>Feed</span>
        </div>
        <div className="nav-item" onClick={() => navigate("/profile")}>
          <i className="icon notifications-icon"></i>
          <span>Profile</span>
        </div>
      </div>
    </>
  );
};

export default BottomNav;
