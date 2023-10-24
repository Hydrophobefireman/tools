import {AuthBridge} from "@hydrophobefireman/flask-jwt-jskit";
import {redirect} from "@hydrophobefireman/ui-lib";

const client = new AuthBridge<any>();

// change these according to your backend
client.routes = {
  loginRoute: "/login",
  refreshTokenRoute: "/refresh",
  initialAuthCheckRoute: "/accounts/me",
};
client.onLogout = () => redirect("/login");

const {useAllAuthState, useIsLoggedIn} = client.getHooks();

const requests = client.getHttpClient();

export {useAllAuthState, useIsLoggedIn, requests, client};
