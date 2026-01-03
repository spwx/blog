// Helper function to create SVG elements
function createSVGElement(tag, attributes) {
    const element = document.createElementNS('http://www.w3.org/2000/svg', tag);
    for (const [key, value] of Object.entries(attributes)) {
        element.setAttribute(key, value);
    }
    return element;
}

// Create copy icon SVG
function createCopyIcon() {
    const svg = createSVGElement('svg', {
        width: '16',
        height: '16',
        viewBox: '0 0 16 16',
        fill: 'none'
    });

    const rect = createSVGElement('rect', {
        x: '5',
        y: '5',
        width: '9',
        height: '9',
        rx: '1',
        stroke: 'currentColor',
        'stroke-width': '1.5'
    });

    const path = createSVGElement('path', {
        d: 'M3 10.5V3C3 2.44772 3.44772 2 4 2H10.5',
        stroke: 'currentColor',
        'stroke-width': '1.5',
        'stroke-linecap': 'round'
    });

    svg.appendChild(rect);
    svg.appendChild(path);
    return svg;
}

// Create checkmark icon SVG
function createCheckIcon() {
    const svg = createSVGElement('svg', {
        width: '16',
        height: '16',
        viewBox: '0 0 16 16',
        fill: 'none'
    });

    const path = createSVGElement('path', {
        d: 'M3 8L6.5 11.5L13 4',
        stroke: 'currentColor',
        'stroke-width': '2',
        'stroke-linecap': 'round',
        'stroke-linejoin': 'round'
    });

    svg.appendChild(path);
    return svg;
}

// Initialize all DOM-dependent features
document.addEventListener('DOMContentLoaded', function() {
    // Set current year in footer
    const yearSpan = document.getElementById('current-year');
    if (yearSpan) {
        yearSpan.textContent = new Date().getFullYear();
    }

    // Add copy buttons to code blocks
    const codeBlocks = document.querySelectorAll('.post-content pre.code');

    codeBlocks.forEach(function(codeBlock) {
        // Create wrapper div
        const wrapper = document.createElement('div');
        wrapper.className = 'code-block-wrapper';

        // Create copy button
        const copyButton = document.createElement('button');
        copyButton.className = 'copy-button';
        copyButton.appendChild(createCopyIcon());
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

            // Clipboard API with fallback
            function copyToClipboard(text) {
                if (navigator.clipboard) {
                    return navigator.clipboard.writeText(text);
                } else {
                    // Fallback for older browsers
                    return new Promise(function(resolve, reject) {
                        const textarea = document.createElement('textarea');
                        textarea.value = text;
                        textarea.style.position = 'fixed';
                        textarea.style.opacity = '0';
                        document.body.appendChild(textarea);
                        textarea.select();
                        try {
                            document.execCommand('copy');
                            document.body.removeChild(textarea);
                            resolve();
                        } catch (err) {
                            document.body.removeChild(textarea);
                            reject(err);
                        }
                    });
                }
            }

            copyToClipboard(text).then(function() {
                copyButton.textContent = '';
                copyButton.appendChild(createCheckIcon());
                copyButton.classList.add('copied');
                copyButton.setAttribute('aria-label', 'Copied to clipboard');

                setTimeout(function() {
                    copyButton.textContent = '';
                    copyButton.appendChild(createCopyIcon());
                    copyButton.classList.remove('copied');
                    copyButton.setAttribute('aria-label', 'Copy code to clipboard');
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
                        const currentLink = document.querySelector(`.toc a[href="#${CSS.escape(activeHeading.id)}"]`);
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
            try {
                if (!localStorage.getItem('theme')) {
                    setTheme(e.matches ? 'light' : 'dark');
                }
            } catch (err) {
                // localStorage not available, skip auto-switch
            }
        });
    }
})();

// Create list icon SVG
function createListIcon() {
    const svg = createSVGElement('svg', {
        width: '16',
        height: '16',
        viewBox: '0 0 16 16',
        fill: 'none'
    });

    const line1 = createSVGElement('line', {
        x1: '5', y1: '3', x2: '14', y2: '3',
        stroke: 'currentColor',
        'stroke-width': '2',
        'stroke-linecap': 'round'
    });

    const line2 = createSVGElement('line', {
        x1: '5', y1: '8', x2: '14', y2: '8',
        stroke: 'currentColor',
        'stroke-width': '2',
        'stroke-linecap': 'round'
    });

    const line3 = createSVGElement('line', {
        x1: '5', y1: '13', x2: '14', y2: '13',
        stroke: 'currentColor',
        'stroke-width': '2',
        'stroke-linecap': 'round'
    });

    const circle1 = createSVGElement('circle', {
        cx: '2', cy: '3', r: '1',
        fill: 'currentColor'
    });

    const circle2 = createSVGElement('circle', {
        cx: '2', cy: '8', r: '1',
        fill: 'currentColor'
    });

    const circle3 = createSVGElement('circle', {
        cx: '2', cy: '13', r: '1',
        fill: 'currentColor'
    });

    svg.appendChild(line1);
    svg.appendChild(line2);
    svg.appendChild(line3);
    svg.appendChild(circle1);
    svg.appendChild(circle2);
    svg.appendChild(circle3);
    return svg;
}

// Create close icon SVG
function createCloseIcon() {
    const svg = createSVGElement('svg', {
        width: '16',
        height: '16',
        viewBox: '0 0 16 16',
        fill: 'none'
    });

    const line1 = createSVGElement('line', {
        x1: '3', y1: '3', x2: '13', y2: '13',
        stroke: 'currentColor',
        'stroke-width': '2',
        'stroke-linecap': 'round'
    });

    const line2 = createSVGElement('line', {
        x1: '13', y1: '3', x2: '3', y2: '13',
        stroke: 'currentColor',
        'stroke-width': '2',
        'stroke-linecap': 'round'
    });

    svg.appendChild(line1);
    svg.appendChild(line2);
    return svg;
}

// Table of Contents Toggle Functionality
(function initTocToggle() {
    const toc = document.querySelector('.toc');
    const tocToggle = document.querySelector('.toc-toggle');
    if (!toc || !tocToggle) return;

    // Create backdrop element
    const backdrop = document.createElement('div');
    backdrop.className = 'toc-backdrop';
    document.body.appendChild(backdrop);

    // Check if mobile view
    function isMobile() {
        return window.matchMedia('(max-width: 1200px)').matches;
    }

    // Get saved state from localStorage
    function getTocState() {
        try {
            const saved = localStorage.getItem('toc-visible');
            if (saved !== null) {
                return saved !== 'false';
            }
            // Default: visible on desktop, hidden on mobile
            return !isMobile();
        } catch (e) {
            return !isMobile(); // Default: visible on desktop, hidden on mobile
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
            tocToggle.textContent = '';
            tocToggle.appendChild(createCloseIcon());
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
            tocToggle.textContent = '';
            tocToggle.appendChild(createListIcon());
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
        if (isMobile()) {
            tocToggle.appendChild(createListIcon());
        }
    } else {
        // If TOC is open on mobile, show close icon
        if (isMobile()) {
            tocToggle.appendChild(createCloseIcon());
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
        // Only trigger if not in an input/textarea/contenteditable
        if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA' || e.target.isContentEditable) return;

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
                tocToggle.textContent = '';
                tocToggle.appendChild(createListIcon());
            } else {
                // Switched to mobile - update icon based on TOC state
                const isHidden = toc.classList.contains('hidden');
                tocToggle.textContent = '';
                tocToggle.appendChild(isHidden ? createListIcon() : createCloseIcon());
            }
        }, 250);
    });
})();

