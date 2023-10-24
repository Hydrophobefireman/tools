import {NotFound} from "@/pages/404";
import {Router as KitRouter} from "@kit/router";

import routes from "./_routes";

export function Router() {
  return <KitRouter paths={routes} NotFoundComponent={NotFound} />;
}
