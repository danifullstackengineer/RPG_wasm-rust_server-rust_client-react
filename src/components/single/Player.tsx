import React, { useEffect, useRef, useState } from "react";
import styles from "../../styles/single/Player.module.scss";
import song from "../../assets/music/TokyoGhoul.mp3";
import { TokyoGhoul } from "../../assets/music/credit";
import { BiSkipNext, BiSkipPrevious } from "react-icons/bi";
import { BsFillPlayFill, BsFillPauseFill } from "react-icons/bs";

const btnStyle = {
  width: 50,
  height: 50,
  fontFamily: "pixelated",
};

const Player = () => {
  const [isPlaying, setIsPlaying] = useState<boolean>(false);

  const musicRef = useRef<HTMLAudioElement>(null);
  const sliderProgressRef = useRef<HTMLDivElement>(null);

  const handlePlay = () => {
    setIsPlaying(!isPlaying);
  };

  useEffect(() => {
    if (musicRef.current) {
      document.documentElement.style.setProperty(
        "--slideProgressBarMusicDuration",
        musicRef.current.duration + "s"
      );
    }
  }, [musicRef.current]);

  useEffect(() => {
    if (musicRef.current && isPlaying) {
      musicRef.current.play();
    } else if (musicRef.current) {
      musicRef.current.pause();
    }
  }, [isPlaying]);

  return (
    <div className={styles.player}>
      <div className={styles.player__current}>{TokyoGhoul}</div>
      <div className={styles.player__slider}>
        <audio src={song} ref={musicRef} loop={true}></audio>
        <div
          className={`${styles.player__slider__progress} ${
            !isPlaying ? styles.player__slider__progress__inactive : ""
          }`}
          ref={sliderProgressRef}
        ></div>
      </div>
      <div className={styles.player__btns}>
        <button>
          <BiSkipPrevious style={btnStyle} />
        </button>
        <button onClick={handlePlay}>
          {isPlaying ? (
            <BsFillPauseFill style={btnStyle} />
          ) : (
            <BsFillPlayFill style={{ ...btnStyle, color: "red" }} />
          )}
        </button>
        <button>
          <BiSkipNext style={btnStyle} />
        </button>
      </div>
    </div>
  );
};

export default Player;
