import React, { useState } from "react";
import { BrowserRouter as Router, Route, Switch, Redirect } from "react-router-dom";
import Header from "./components/Header";
import Stake from "./components/Stake";
import Unstake from "./components/Unstake";
import ClaimRewards from "./components/ClaimRewards";
import AdminDashboard from "./components/AdminDashboard";
import Layout from "./components/Layout";
import { useWallet } from "@solana/wallet-adapter-react";

const admin = process.env.REACT_APP_ADMIN_ADDRESS;
function App() {
  const wallet = useWallet();
  // const currentAddress = wallet?.publicKey?.toBase58() || null;
  const currentAddress = "A3jeuMkQBNwZoSqjzdVjNFJhvhRNFbua7LigR3gZ3TLZ";

  return (
    <Router>
      <Layout>
        <Header />
        <hr className="bg-white" />
        <Switch>
          <Route path="/stake" component={Stake} />
          <Route path="/unstake" component={Unstake} />
          <Route path="/claim-rewards" component={ClaimRewards} />
          <PrivateRoute path="/admin-dashboard" component={AdminDashboard} address={currentAddress} />
          <Redirect from="/" to="/stake" />
        </Switch>
      </Layout>
    </Router>
  );
}

function PrivateRoute({ component: Component, address, ...rest }) {
  return <Route {...rest} render={(props) => (address == admin ? <Component {...props} /> : <Redirect to="/" />)} />;
}

export default App;
