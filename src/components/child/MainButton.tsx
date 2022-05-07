import React from 'react'
import { Link } from 'react-router-dom';
import styles from '../../styles/child/MainButton.module.scss';

const MainButton = ({text, link, selected}: {text: string, link: string, selected: boolean}) => {
  return (
    <Link className={`${styles.mainButton} ${selected ? styles.mainButton__selected : ""}`} to={link}>{text}</Link>
  )
}

export default React.memo(MainButton);