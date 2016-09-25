extern crate byteorder;
use self::byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Read, Error};
use std::fs::File;

pub fn read_string_iso<T: Read>(buf: &mut T) -> Result<String, Error> {
	let len = (try!(buf.read_u32::<LittleEndian>())) as usize;
	let mut iso = Vec::with_capacity(len);
	iso.resize(len, 0);
	
	println!("{}", len);
	
	try!(buf.read_exact(&mut iso));
	
	let mut str = String::with_capacity(len);
	for byte in iso {
		str.push(byte as char);
	}
	
	Ok(str)
}

#[derive(Debug)]
enum BonusType {
	Attack,
	HitPoints,
	AttackSpeed,
	Armor,
	Range,
	Speed,
	FlightTime,
	AreaDamage,
	FoodCollectionFromHuntingAndForaging,
	WoodCollection,
	GoldCollection,
	FoodCollectionFromFarm,
	StoneCollection,
	IronCollection,
	BuildTime,
	ConversionResistance,
	Fishing,
	Repair,
	AreaEffect,
	MountainCombat,
	PopulationLimit,
	AreaConversion,
	Cost,
	
	NightTimeLos,
	Slavery,
	CapitolUpgrade,
	Heroes,
	Placeholder1,
	Placeholder2,
	Placeholder3,
	Placeholder4,
	Placeholder5,
	Placeholder6,
	Placeholder7,
	Placeholder8,
	Placeholder9,
	Placeholder10,
}

impl From<u32> for BonusType {
	fn from(id: u32) -> BonusType {
		match id {
			1 => BonusType::Attack,
			2 => BonusType::HitPoints,
			3 => BonusType::AttackSpeed,
			4 => BonusType::Armor,
			5 => BonusType::Range,
			6 => BonusType::Speed,
			7 => BonusType::FlightTime,
			8 => BonusType::AreaDamage,
			9 => BonusType::FoodCollectionFromHuntingAndForaging,
			10 => BonusType::WoodCollection,
			11 => BonusType::GoldCollection,
			12 => BonusType::FoodCollectionFromFarm,
			13 => BonusType::StoneCollection,
			14 => BonusType::IronCollection,
			15 => BonusType::BuildTime,
			16 => BonusType::ConversionResistance,
			17 => BonusType::Fishing,
			18 => BonusType::Repair,
			19 => BonusType::AreaEffect,
			20 => BonusType::MountainCombat,
			21 => BonusType::PopulationLimit,
			22 => BonusType::AreaConversion,
			23 => BonusType::Cost,
			24 => BonusType::NightTimeLos,
			25 => BonusType::Slavery,
			26 => BonusType::CapitolUpgrade,
			27 => BonusType::Heroes,
			28 => BonusType::Placeholder1,
			29 => BonusType::Placeholder2,
			30 => BonusType::Placeholder3,
			31 => BonusType::Placeholder4,
			32 => BonusType::Placeholder5,
			33 => BonusType::Placeholder6,
			34 => BonusType::Placeholder7,
			35 => BonusType::Placeholder8,
			36 => BonusType::Placeholder9,
			37 => BonusType::Placeholder10,
			x => panic!("Unknown: {}", x)
		}
	}
}

#[derive(Debug)]
struct Bonus {
	bonus_type: BonusType,
	/// Warning: This is not the dbcivilization id, but the index in the file.
	/// Proboably a bug that was never fixed.
	target: u32
}

impl From<u32> for Bonus {
	fn from(id: u32) -> Bonus {
		Bonus {
			bonus_type: BonusType::from(id&63),
			target: id>>6
		}
	}
}

pub fn main() {
	let mut scn = File::open("/home/coderbot/eclipse/workspace/EmpireEarthReverse/extract/AllBonuses.civ").expect("File open failed");
	println!("{:?}", read_string_iso(&mut scn));
	for _ in 0..(scn.read_u32::<LittleEndian>().unwrap()) {
		let bonus: u32 = scn.read_u32::<LittleEndian>().unwrap();
		println!("{:?}", Bonus::from(bonus));
	}
}