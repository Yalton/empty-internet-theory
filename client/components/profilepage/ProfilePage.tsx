import React from "react";
import BottomNav from "../common/BottomNav";
import Post from "../common/Post";
import ProfileInfo from "../common/ProfileInfo";
import "../App.css";

const ProfilePage = () => {
  const userProfile = {
    displayName: "Jerome PogChampion",
    username: "JeromeJorganson",
    bio: "Passionate gamer and streamer. Love to share my gaming moments.",
    profilePicture: "/path-to-profile-pic.jpg", // Replace with actual path to image
  };

  const posts = [
    {
      displayName: "Jerome PogChampion",
      username: "JeromeJorganson",
      timePosted: "2s",
      content:
        "lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
    },
    {
      displayName: "Jerome PogChampion",
      username: "JeromeJorganson",
      timePosted: "2s",
      content:
        "lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
    },
    {
      displayName: "Jerome PogChampion",
      username: "JeromeJorganson",
      timePosted: "2s",
      content:
        "lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
    },
    {
      displayName: "Jerome PogChampion",
      username: "JeromeJorganson",
      timePosted: "2s",
      content:
        "lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
    },
    {
      displayName: "Jerome PogChampion",
      username: "JeromeJorganson",
      timePosted: "2s",
      content:
        "lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
    },
    {
      displayName: "Jerome PogChampion",
      username: "JeromeJorganson",
      timePosted: "2s",
      content:
        "lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
    },
    {
      displayName: "Jerome PogChampion",
      username: "JeromeJorganson",
      timePosted: "2s",
      content:
        "lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
    },
  ];

  return (
    <div className="app">
      <ProfileInfo
        profilePicture={userProfile.profilePicture}
        displayName={userProfile.displayName}
        username={userProfile.username}
        bio={userProfile.bio}
      />
      <div className="feed">
        {posts.map((post, index) => (
          <Post
            key={index}
            displayName={post.displayName}
            username={post.username}
            timePosted={post.timePosted}
            content={post.content}
          />
        ))}
      </div>
      <BottomNav />
    </div>
  );
};

export default ProfilePage;
