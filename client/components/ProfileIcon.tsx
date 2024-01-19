import Pikachu from "@assets/image.png";

const ProfileIcon = () => (
	<div className="rounded-full">
		<img
			src={Pikachu}
			alt={"Pikachu surprised face"}
			className={"w-8 h-8 bg-sky-900 rounded-full"}
		/>
	</div>
);

export default ProfileIcon;
