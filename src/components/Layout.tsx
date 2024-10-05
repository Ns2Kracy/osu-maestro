import "tailwindcss/tailwind.css";
import Footer from "./Footer";
import Header from "./Header";

export default function AppLayout() {
	return (
		<div class="min-h-screen flex flex-col">
			<Header />
			<div class="flex flex-1">
				<main class="flex-1 p-6 bg-white">1</main>
			</div>
			<Footer />
		</div>
	);
}
