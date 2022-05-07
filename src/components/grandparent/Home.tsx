import React from "react";
import styles from "../../styles/grandparent/Home.module.scss";
import Player from "./Player";

const Home = () => {
  return (
    <div className={styles.home}>
      <div className={styles.home__container}>
        <Player />
      </div>
    </div>
  );
};

export default Home;
