// Initialize all DOM-dependent features
document.addEventListener('DOMContentLoaded', function() {
    // Set current year in footer
    const yearSpan = document.getElementById('current-year');
    if (yearSpan) {
        yearSpan.textContent = new Date().getFullYear();
    }

    // Add copy buttons to code blocks
    const codeBlocks = document.querySelectorAll('.post-content pre.code');

    // Copy icon SVG
    const copyIconSVG = '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg"><rect x="5" y="5" width="9" height="9" rx="1" stroke="currentColor" stroke-width="1.5"/><path d="M3 10.5V3C3 2.44772 3.44772 2 4 2H10.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>';

    // Checkmark icon SVG for success state
    const checkIconSVG = '<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M3 8L6.5 11.5L13 4" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></svg>';

    codeBlocks.forEach(function(codeBlock) {
        // Create wrapper div
        const wrapper = document.createElement('div');
        wrapper.className = 'code-block-wrapper';

        // Create copy button
        const copyButton = document.createElement('button');
        copyButton.className = 'copy-button';
        copyButton.innerHTML = copyIconSVG;
        copyButton.setAttribute('aria-label', 'Copy code to clipboard');
        copyButton.setAttribute('data-tooltip', 'Copy');

        // Wrap the code block
        codeBlock.parentNode.insertBefore(wrapper, codeBlock);
        wrapper.appendChild(codeBlock);
        wrapper.appendChild(copyButton);

        // Add click handler
        copyButton.addEventListener('click', function() {
            const code = codeBlock.querySelector('code');
            const text = code ? code.textContent : codeBlock.textContent;

            navigator.clipboard.writeText(text).then(function() {
                copyButton.innerHTML = checkIconSVG;
                copyButton.classList.add('copied');

                setTimeout(function() {
                    copyButton.innerHTML = copyIconSVG;
                    copyButton.classList.remove('copied');
                }, 2000);
            }).catch(function(err) {
                console.error('Failed to copy:', err);
            });
        });
    });

    // Table of Contents Active Section Highlighting
    const tocLinks = document.querySelectorAll('.toc a');
    if (tocLinks.length > 0) {
        // Get all headings that have IDs
        const headings = Array.from(document.querySelectorAll('.post-content h1[id], .post-content h2[id], .post-content h3[id], .post-content h4[id], .post-content h5[id], .post-content h6[id]'));

        if (headings.length > 0) {
            // Use IntersectionObserver for better performance than scroll events
            const observerOptions = {
                rootMargin: '-100px 0px -66% 0px',
                threshold: 0
            };

            let activeHeading = null;

            const observer = new IntersectionObserver((entries) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        activeHeading = entry.target;

                        // Remove active class from all links
                        tocLinks.forEach(link => link.classList.remove('active'));

                        // Add active class to current link
                        const currentLink = document.querySelector(`.toc a[href="#${activeHeading.id}"]`);
                        if (currentLink) {
                            currentLink.classList.add('active');
                        }
                    }
                });
            }, observerOptions);

            // Observe all headings
            headings.forEach(heading => observer.observe(heading));
        }
    }
});

// Theme Toggle Functionality
(function initThemeToggle() {
    const toggle = document.getElementById('theme-toggle');
    if (!toggle) return;

    function getCurrentTheme() {
        return document.documentElement.getAttribute('data-theme') || 'dark';
    }

    function setTheme(theme) {
        document.documentElement.setAttribute('data-theme', theme);
        try {
            localStorage.setItem('theme', theme);
        } catch (e) {
            console.warn('Could not save theme preference:', e);
        }
    }

    function toggleTheme() {
        const current = getCurrentTheme();
        const next = current === 'dark' ? 'light' : 'dark';
        setTheme(next);
    }

    toggle.addEventListener('click', toggleTheme);

    // Listen for system theme changes
    if (window.matchMedia) {
        const mediaQuery = window.matchMedia('(prefers-color-scheme: light)');
        mediaQuery.addEventListener('change', (e) => {
            // Only auto-switch if user hasn't manually set a preference
            if (!localStorage.getItem('theme')) {
                setTheme(e.matches ? 'light' : 'dark');
            }
        });
    }
})();

// Table of Contents Toggle Functionality
(function initTocToggle() {
    const toc = document.querySelector('.toc');
    const tocToggle = document.querySelector('.toc-toggle');
    if (!toc || !tocToggle) return;

    // Create backdrop element
    const backdrop = document.createElement('div');
    backdrop.className = 'toc-backdrop';
    document.body.appendChild(backdrop);

    // Icon SVGs
    const listIcon = `<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
        <line x1="5" y1="3" x2="14" y2="3" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        <line x1="5" y1="8" x2="14" y2="8" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        <line x1="5" y1="13" x2="14" y2="13" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        <circle cx="2" cy="3" r="1" fill="currentColor"/>
        <circle cx="2" cy="8" r="1" fill="currentColor"/>
        <circle cx="2" cy="13" r="1" fill="currentColor"/>
    </svg>`;

    const closeIcon = `<svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
        <line x1="3" y1="3" x2="13" y2="13" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        <line x1="13" y1="3" x2="3" y2="13" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
    </svg>`;

    // Check if mobile view
    function isMobile() {
        return window.matchMedia('(max-width: 1200px)').matches;
    }

    // Get saved state from localStorage
    function getTocState() {
        try {
            return localStorage.getItem('toc-visible') !== 'false';
        } catch (e) {
            return true; // Default to visible
        }
    }

    // Save state to localStorage
    function saveTocState(visible) {
        try {
            localStorage.setItem('toc-visible', visible);
        } catch (e) {
            console.warn('Could not save TOC state:', e);
        }
    }

    // Open TOC
    function openToc() {
        if (isMobile()) {
            backdrop.classList.add('visible');
            tocToggle.innerHTML = closeIcon;
        }
        toc.classList.remove('hidden');
        tocToggle.classList.remove('toc-hidden');
        saveTocState(true);
    }

    // Close TOC
    function closeToc() {
        toc.classList.add('hidden');
        tocToggle.classList.add('toc-hidden');
        if (isMobile()) {
            backdrop.classList.remove('visible');
            tocToggle.innerHTML = listIcon;
        }
        saveTocState(false);
    }

    // Toggle TOC visibility
    function toggleToc() {
        const isHidden = toc.classList.contains('hidden');
        if (isHidden) {
            openToc();
        } else {
            closeToc();
        }
    }

    // Initialize TOC state
    if (!getTocState()) {
        toc.classList.add('hidden');
        tocToggle.classList.add('toc-hidden');
    } else {
        // If TOC is open on mobile, show close icon
        if (isMobile()) {
            tocToggle.innerHTML = closeIcon;
        }
    }

    // Mark TOC as initialized to make it visible (if not hidden)
    toc.classList.add('initialized');

    // Button click handler
    tocToggle.addEventListener('click', toggleToc);

    // Backdrop click handler - close TOC
    backdrop.addEventListener('click', closeToc);

    // Close TOC when clicking a link (mobile only)
    const tocLinks = toc.querySelectorAll('a');
    tocLinks.forEach(link => {
        link.addEventListener('click', function() {
            if (isMobile()) {
                closeToc();
            }
        });
    });

    // Keyboard shortcuts
    document.addEventListener('keydown', function(e) {
        // Only trigger if not in an input/textarea
        if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;

        // Escape key - close TOC
        if (e.key === 'Escape' && !toc.classList.contains('hidden')) {
            e.preventDefault();
            closeToc();
            return;
        }

        // Only trigger if no modifier keys are pressed (to allow Ctrl+C, Cmd+C, etc.)
        if (e.ctrlKey || e.metaKey || e.altKey || e.shiftKey) return;

        // C key - toggle TOC
        if (e.key === 'c' || e.key === 'C') {
            e.preventDefault();
            toggleToc();
        }
    });

    // Handle window resize - clean up backdrop and icon if switching from mobile to desktop
    let resizeTimer;
    window.addEventListener('resize', function() {
        clearTimeout(resizeTimer);
        resizeTimer = setTimeout(function() {
            if (!isMobile()) {
                backdrop.classList.remove('visible');
                tocToggle.innerHTML = listIcon;
            } else {
                // Switched to mobile - update icon based on TOC state
                const isHidden = toc.classList.contains('hidden');
                tocToggle.innerHTML = isHidden ? listIcon : closeIcon;
            }
        }, 250);
    });
})();

