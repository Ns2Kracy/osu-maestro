import { DarkModeToggle } from "./DarkModeToggle";

export default function Header() {
	return (
		<header class="bg-white shadow p-4 flex items-center">
			{/* <div class="text-2xl font-bold">osu! Maestro</div> */}
			{/* use svg logo in public/logo.svg */}
			{/* biome-ignore lint/a11y/useAltText: <explanation> */}
			<img src="/logo.svg" class="w-8 h-8" />
			{/* </explanation> */}
			<DarkModeToggle />
		</header>
	);
}
