import React from "react";
import EventListener from "react-event-listener";
import styles from "../../styles/grandparent/Player.module.scss";

const Player = () => {
  const handleKeyDownPlayer = (e: KeyboardEvent): void => {
      
  };

  return (
    <>
      <EventListener
        target={"document"}
        onKeyDown={(ev) => handleKeyDownPlayer(ev)}
      />
      <div className={styles.player}></div>
    </>
  );
};

export default React.memo(Player);
