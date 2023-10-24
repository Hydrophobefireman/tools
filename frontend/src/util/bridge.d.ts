import {AuthBridge} from "@hydrophobefireman/flask-jwt-jskit";
declare const client: AuthBridge<any>;
declare const useAllAuthState: () => [
    import("@hydrophobefireman/flask-jwt-jskit/dist/src/types").AppAuthState<any>,
    import("statedrive").SetSharedState<
      import("@hydrophobefireman/flask-jwt-jskit/dist/src/types").AppAuthState<any>
    >,
  ],
  useIsLoggedIn: () => boolean;
declare const requests: import("@hydrophobefireman/flask-jwt-jskit").HttpClient;
export {useAllAuthState, useIsLoggedIn, requests, client};
