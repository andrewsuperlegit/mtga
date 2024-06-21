import {useState} from "react";
import Image from "next/image";
import clsx from 'clsx';

import React from 'react'
import type { RootState } from '../../app/store'
import { useSelector, useDispatch } from 'react-redux'
import { tap } from './../Actions';

export default function Card({card, keyName}){
	const [isActive, setActive] = useState(false);
	// setActive(behavior.battlefieldBehavior.isTapped);
	const game = useSelector((state: RootState) => state.game);
	const dispatch = useDispatch()

	function getClassNames(){
		return clsx(
		`${card.color}-color card`, {
			'tapped': card.behavior.battlefieldBehavior.isTapped === true,
			'untapped': card.behavior.battlefieldBehavior.isTapped === false,
			'summon-sick': card.behavior.battlefieldBehavior.isSummonSick === true
		}
	);
	}

	function tap(){
		card.behavior.tapForMana(dispatch, card);
	}

	return(
		<>
		<section className={
			getClassNames()
		}
		onClick={tap}
		>
			<header className="card-header">
				<h1>{card.name}, {keyName}</h1>
				<h2>{card.cost}</h2>
			</header>
			<div className="image-wrapper">
				<Image
				className="relative dark:drop-shadow-[0_0_0.3rem_#ffffff70] dark:invert"
				src="/next.svg"
				alt="Next.js Logo"
				width={180}
				height={37}/>
			</div>
			<p>{card.description}</p>
		</section>
		</>
	)
}
