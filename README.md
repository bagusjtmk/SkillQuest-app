SkillQuest
SkillQuest - Gamified Academic Reputation System on Soroban
Project Description
SkillQuest is a decentralized gamification platform built on the Stellar blockchain using Soroban SDK. It transforms academic activities into a transparent, verifiable, and engaging experience. Students earn XP, level up, collect badges, and build a digital reputation — all stored immutably on-chain through smart contract functions, eliminating the need for centralized academic record systems.
The system enables universities and lecturers to issue XP rewards and badges for real academic activities such as attending seminars, winning hackathons, and completing workshops. Each student's achievement is uniquely recorded on-chain, ensuring authenticity and preventing falsification.
Project Vision
Our vision is to revolutionize academic engagement and credentialing in higher education by:

Decentralizing Achievement Records: Moving academic credentials from centralized databases to a global, distributed blockchain
Ensuring Verifiability: Empowering students with tamper-proof, on-chain proof of their academic accomplishments
Guaranteeing Transparency: Providing an immutable leaderboard and reputation score that cannot be manipulated by any party
Enhancing Motivation: Leveraging game mechanics — XP, levels, and badges — to drive genuine academic participation
Building Trustless Systems: Creating a platform where achievement integrity is guaranteed by code, not by institution promises

We envision a future where a student's academic reputation is truly self-sovereign — portable, verifiable, and owned entirely by the individual.
Key Features
1. Student Identity & Profile

Register a unique on-chain identity linked to a Stellar wallet
Persistent student profile storing XP, level, reputation score, and badge count
Automated level calculation based on XP using a square-root scaling formula
Reputation score dynamically recalculated on every achievement update

2. XP & Leveling System

Earn XP through verified academic activities issued by admins
Level progression scaled from Level 1 to Level 10 using the formula: Level = √(XP / 100) + 1
Anti-cheat by design: students cannot self-claim XP — only authorized admins can issue rewards
Real-time level and reputation update on every XP addition

3. Badge Collection System

Collect achievement badges across four rarity tiers: Common, Rare, Epic, and Legendary
Each badge is uniquely created by admins with a name, description, rarity, and XP requirement
Rare, Epic, and Legendary badges contribute bonus reputation points (+20 each)
Badge ownership is recorded on-chain and prevents duplicate claims

4. Quest & Event System

Admins create academic quests (events) with configurable XP rewards
Students complete events and receive XP automatically upon admin verification
Each student can only claim each event once — enforced by on-chain claim tracking
Supports a wide range of activities: seminars, workshops, hackathons, and coursework

5. On-Chain Leaderboard

Global leaderboard ranking all registered students by XP
Real-time standings calculated directly from on-chain data
Configurable limit to retrieve top N students
Encourages healthy academic competition across the campus

6. Admin Management System

Deployer wallet is automatically set as the first admin upon contract initialization
Admins can authorize additional admins for distributed management
All XP additions, badge issuances, and event completions require admin authentication
Role-based access control enforced natively via Soroban's require_auth()

Contract Details

Contract Name: SkillQuest
Network: Stellar Testnet (Soroban)
Language: Rust (Soroban SDK v21)
Contract Address: (deploy contract and update here)

Contract Functions
FunctionAccessDescriptioninitialize(deployer)One-timeInitialize contract and set first adminregister(wallet, username)PublicRegister a new student identity on-chainget_student(wallet)PublicRetrieve a student's full profileis_registered(wallet)PublicCheck if a wallet is already registeredadd_xp(admin, student, amount)AdminAward XP to a studentcreate_badge(admin, name, desc, rarity, xp_req)AdminCreate a new badgeunlock_badge(admin, student, badge_id)AdminIssue a badge to a studenthas_badge(student, badge_id)PublicCheck if a student owns a specific badgeget_badge(badge_id)PublicGet badge metadatacreate_event(admin, title, xp_reward)AdminCreate a new academic quest/eventcomplete_event(admin, student, event_id)AdminMark event complete and award XPget_leaderboard(limit)PublicGet top N students ranked by XPadd_admin(caller, new_admin)AdminGrant admin role to another walletis_admin(wallet)PublicCheck if a wallet has admin privileges
Future Scope
Short-Term Enhancements

Daily Quest System: Reset-able daily challenges to incentivize consistent participation
Seasonal Leaderboard: Semester-based ranking snapshots to reward top performers per period
Badge Marketplace: Allow students to showcase rare badges on a public gallery

Medium-Term Development

Guild System: Group students into academic guilds (e.g., AI Guild, Blockchain Guild) with shared XP pools

Guild leaderboard separate from individual rankings
Guild quests with collaborative XP rewards
Guild reputation based on member achievements


NFT Badge Integration: Mint Legendary badges as Stellar-based NFTs for true digital ownership
Cross-Institution Support: Extend the system to multiple campuses with federated admin roles

Long-Term Vision

Decentralized Credential Verification: Allow employers to verify a student's on-chain academic reputation directly
Zero-Knowledge Proof Privacy: Let students selectively disclose achievements without revealing full profile data
DAO Governance: Community-driven addition of new badge types and quest categories
AI-Powered Quest Suggestions: Recommend academic activities based on a student's current level and badge collection
Cross-Chain Reputation: Bridge student reputation scores to other blockchain ecosystems
Identity Management: Integration with decentralized identity (DID) systems for verified student credentials

Enterprise & Institutional Features

University Dashboard: Admin panel for bulk XP distribution and event management
Immutable Academic Logging: Time-locked achievement records for official transcript use
Automated Reporting: Periodic on-chain reports of campus engagement statistics
Multi-Language Support: Expand accessibility with full internationalization support


Technical Requirements

Soroban SDK v21
Rust programming language
Stellar blockchain network (Testnet / Mainnet)
Freighter Wallet (for frontend integration)

Getting Started
Deploy the smart contract to Stellar's Soroban network and interact with it using the main functions:

initialize() — Set up the contract and assign the first admin
register() — Register a student wallet with a username
add_xp() — Award XP to a student for completing activities
create_badge() — Define a new achievement badge
unlock_badge() — Issue a badge to a qualifying student
create_event() — Create a new academic quest with XP reward
complete_event() — Verify participation and distribute XP
get_leaderboard() — Fetch the top-ranked students on-chain

**Stellar Notes DApp** - Securing Your Thoughts on the Blockchain
ID SMART CONTRACT: CBEN7HL2YXD5LK4QMRLEBHRJ7OKEX6BURRAWUZEFINFB3NOOHJKUTHK4