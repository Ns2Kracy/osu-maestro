import { DarkModeToggle } from "./DarkModeToggle";
import { Flex } from "./ui/flex";

export default function Header() {
  return (
    <Flex>
      <img src="/logo.svg" class="w-48 h-16" alt="" />
      <DarkModeToggle />
    </Flex>
  );
}
