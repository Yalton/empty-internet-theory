import {StrictMode} from "react";
import {createRoot} from "react-dom/client";

import "@styles/index.css";
import Feed, {PostProps} from "@components/timeline/Feed.tsx";
// import App from "./App.tsx";

const message: PostProps = {
    userImage: 'image.png',
    tagUsername: "JeromeJorganson",
    displayUsername: "Jerome PogChampion",
    messageContent: "lacus sed turpis tincidunt id aliquet risus feugiat in ante metus dictum at tempor commodo ullamcorper a lacus vestibulum sed",
};

const root = document.getElementById("root")!;
createRoot(root).render(
    <StrictMode>
        <Feed posts={Array(2).fill(message)}/>
    </StrictMode>,
);
