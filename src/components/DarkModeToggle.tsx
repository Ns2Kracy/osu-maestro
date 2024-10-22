import { useColorMode } from "@kobalte/core";
import { Moon, Sun } from "lucide-solid";
import { Show } from "solid-js";

import { Button } from "~/components/ui/button";
import { Toggle } from "~/components/ui/toggle";

export function DarkModeToggle() {
	const { setColorMode } = useColorMode();

	return (
		<Toggle>
			{(state) => (
				<Button
					variant="ghost"
					size="sm"
					class="w-9 px-0"
					onClick={() => {
						if (state.pressed()) {
							setColorMode("light");
						} else {
							setColorMode("dark");
						}
					}}
				>
					<Show
						when={state.pressed()}
						fallback={
							<Sun class="size-6 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
						}
					>
						<Moon class="absolute size-6 rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
					</Show>
				</Button>
			)}
		</Toggle>
	);
}
