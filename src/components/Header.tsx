import { DarkModeToggle } from "./DarkModeToggle";
import { Flex } from "./ui/flex";

export default function Header() {
  return (
    <Flex>
      {/* <header class="shadow p-4 flex items-center"> */}
      <img src="/logo.svg" class="w-16 h-16" alt="" />
      <DarkModeToggle />
      {/* </header> */}
    </Flex>
  );
}
