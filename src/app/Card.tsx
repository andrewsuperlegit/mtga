import {useState} from "react";
import Image from "next/image";
import clsx from 'clsx';

import React from 'react'
import type { RootState } from '../../app/store'
import { useSelector, useDispatch } from 'react-redux'
import { tap } from './../Actions';

export default function Card({name, color, cost, description, behavior, keyName}){
	const [isActive, setActive] = useState(false);
	// setActive(behavior.battlefieldBehavior.isTapped);
	const game = useSelector((state: RootState) => state.game);
	const dispatch = useDispatch()

	function getClassNames(){
		return clsx(
		`${color}-color card`, {
			'tapped': behavior.battlefieldBehavior.isTapped === true,
			'untapped': behavior.battlefieldBehavior.isTapped === false,
			'summon-sick': behavior.battlefieldBehavior.isSummonSick === true
		}
	);
	}

	function tap(){
		behavior.tapForMana(dispatch);
		// setActive(!isActive);
		// dispatch(tap(data))
	}

	return(
		<>
		<section className={
			getClassNames()
		}
		onClick={tap}
		>
			<header className="card-header">
				<h1>{name}, {keyName}</h1>
				<h2>{cost}</h2>
			</header>
			<div className="image-wrapper">
				<Image
				className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
				src="/next.svg"
				alt="Next.js Logo"
				width={180}
				height={37}/>
			</div>
			<p>{description}</p>
		</section>
		</>
	)
}
