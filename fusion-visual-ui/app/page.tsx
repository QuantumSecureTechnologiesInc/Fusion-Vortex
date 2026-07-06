'use client';

import React, { useState, useEffect } from 'react';
import { Terminal, Cpu, Zap, Activity, Folder, Play, Layers } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import styles from './page.module.css';

const fileExplorerItems = ['src', 'main.fsn', 'utils.fsn', 'tests', 'Cargo.toml', 'Flux.lock'];

const actionCards = [
    { icon: <Play size={20} />, label: "Run Tests", desc: "Execute full suite" },
    { icon: <Layers size={20} />, label: "Compile", desc: "Build release binary" },
    { icon: <Folder size={20} />, label: "Deliver", desc: "Package for deployment" },
];

export default function Home() {
  const [intent, setIntent] = useState('');
  const [status, setStatus] = useState<string[]>([]);
  const [isCompiling, setIsCompiling] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!intent.trim()) return;

    setIsCompiling(true);
    setStatus(prev => [...prev, `> [INTENT_RECEIVED] ${intent}`]);
    setStatus(prev => [...prev, `> [ANALYZING] Intent parsing via Fusion AI Core...`]);

    try {
        // Mock backend call for now
        // const res = await fetch('http://localhost:3000/api/intent', ...);
        
        setTimeout(() => {
            setStatus(prev => [...prev, `> [FLUX_RESOLVE] Optimizing dependencies...`]);
        }, 800);

        setTimeout(() => {
            setStatus(prev => [...prev, `> [COMPILER] Target: Quantum-Native (x86_64-fusion-unknown)`]);
            setStatus(prev => [...prev, `> [SUCCESS] Processed intent: ${intent}`]);
            // setIsCompiling(false); // keep showing log for effect
        }, 2000);

    } catch (err) {
        setStatus(prev => [...prev, `> [ERROR] Connection failed.`]);
        setIsCompiling(false);
    }
    setIntent('');
  };

  return (
    <main className={styles.container}>
      {/* Header */}
      <header className={styles.header}>
        <div className={styles.brand}>
            <Zap className="text-cyan-400" size={24} color="#00f3ff" />
            <h1 className={styles.title}>
                FUSION COMPILER <span className={styles.version}>v0.2.0-beta</span>
            </h1>
        </div>
        <div className={styles.statusBar}>
            <div className={styles.statusItem}><Cpu size={14} /> IDLE</div>
            <div className={styles.statusItem}><Activity size={14} /> ONLINE</div>
        </div>
      </header>

      <div className={styles.contentWrapper}>
        {/* Sidebar */}
        <aside className={styles.sidebar}>
            <div className={styles.sidebarHeader}>Project Explorer</div>
            <div className={styles.fileExplorer}>
                {fileExplorerItems.map((item, i) => (
                    <div key={i} className={styles.fileItem}>
                        <Folder size={14} color="#bd00ff" />
                        {item}
                    </div>
                ))}
            </div>
            <div className={styles.fluxStatus}>
                <div className={styles.fluxActive}>
                    <div className={styles.pulse} />
                    Flux Resolve Active
                </div>
            </div>
        </aside>

        {/* Main Content */}
        <div className={styles.mainContent}>
            <div className={styles.ambientBg} />

            <div style={{zIndex: 1, width: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center'}}>
                <AnimatePresence mode="wait">
                    {!isCompiling ? (
                        <motion.div 
                            key="input"
                            initial={{ opacity: 0, y: 20 }}
                            animate={{ opacity: 1, y: 0 }}
                            exit={{ opacity: 0, y: -20 }}
                            className={styles.intentWrapper}
                        >
                            <form onSubmit={handleSubmit}>
                                <div className={styles.intentBar}>
                                    <div className={styles.intentIcon}>
                                        <Terminal size={20} />
                                    </div>
                                    <input 
                                        type="text" 
                                        value={intent}
                                        onChange={(e) => setIntent(e.target.value)}
                                        placeholder="Enter your intent (e.g., 'Setup robust machine learning pipeline')..." 
                                        className={styles.intentInput}
                                        autoFocus
                                    />
                                    <button type="submit" className={styles.executeBtn}>
                                        EXECUTE
                                    </button>
                                </div>
                            </form>
                            <div className={styles.actionsGrid}>
                                {actionCards.map((action, i) => (
                                    <div key={i} className={styles.actionCard}>
                                        <div className={styles.actionIcon}>{action.icon}</div>
                                        <div className={styles.actionLabel}>{action.label}</div>
                                        <div className={styles.actionDesc}>{action.desc}</div>
                                    </div>
                                ))}
                            </div>
                        </motion.div>
                    ) : (
                        <motion.div 
                            key="console"
                            initial={{ opacity: 0, scale: 0.95 }}
                            animate={{ opacity: 1, scale: 1 }}
                            exit={{ opacity: 0, scale: 0.95 }}
                            className={styles.console}
                        >
                            <div className={styles.consoleOutput}>
                                {status.map((line, i) => (
                                    <motion.div 
                                        key={i}
                                        initial={{ x: -10, opacity: 0 }}
                                        animate={{ x: 0, opacity: 1 }}
                                        className={`${styles.logLine} ${
                                            line.includes('[ERROR]') ? styles.logError : 
                                            line.includes('[SUCCESS]') ? styles.logSuccess : 
                                            styles.logInfo
                                        }`}
                                    >
                                        {line}
                                    </motion.div>
                                ))}
                            </div>
                             <div className={styles.progressBarBg}>
                                <motion.div 
                                    className={styles.progressBar}
                                    initial={{ width: "0%" }}
                                    animate={{ width: "100%" }}
                                    transition={{ duration: 2.5 }}
                                />
                            </div>
                        </motion.div>
                    )}
                </AnimatePresence>
            </div>
        </div>
      </div>
    </main>
  );
}
