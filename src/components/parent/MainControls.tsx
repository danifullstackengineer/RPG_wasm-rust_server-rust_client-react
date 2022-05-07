import React from 'react'
import styles from '../../styles/parent/MainControls.module.scss';
import MainTermsAndConditions from '../child/MainTermsAndConditions';

const MainControls = () => {
  return (
    <div className={styles.mainControls}>
      <MainTermsAndConditions/>
    </div>
  )
}

export default React.memo(MainControls);