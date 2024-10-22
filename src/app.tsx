import {
  ColorModeProvider,
  ColorModeScript,
  cookieStorageManagerSSR,
} from "@kobalte/core";
import { Router, useLocation, useNavigate } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { Suspense } from "solid-js";
import { isServer } from "solid-js/web";
import { Toaster } from "solid-sonner";
import { getCookie } from "vinxi/http";

import "./app.css";
import AppLayout from "./components/Layout";

function createQueryClient() {
  return new QueryClient({});
}

function getServerCookies() {
  "use server";
  const colorMode = getCookie("kb-color-mode");
  return colorMode ? `kb-color-mode=${colorMode}` : "";
}

export default function App() {
  const queryClient = createQueryClient();
  const storageManager = cookieStorageManagerSSR(
    isServer ? getServerCookies() : document.cookie,
  );

  return (
    <QueryClientProvider client={queryClient}>
      <Router
        root={(props) => {
          const navigate = useNavigate();
          const location = useLocation();

          return (
            <>
              <ColorModeScript storageType={storageManager.type} />
              <ColorModeProvider storageManager={storageManager}>
                <AppLayout />
                <Toaster />
                <Suspense>{props.children}</Suspense>
              </ColorModeProvider>
            </>
          );
        }}
      >
        <FileRoutes />
      </Router>
    </QueryClientProvider>
  );
}
