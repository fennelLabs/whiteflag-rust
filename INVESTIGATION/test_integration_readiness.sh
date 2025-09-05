#!/bin/bash

# Test Integration Readiness for Sr25519 JWT/JWS Implementation
# This script validates the current state and readiness for implementing
# sr25519 signatures within JWT/JWS structure for Whiteflag integration

set -e  # Exit on any error

echo "🔍 Whiteflag Sr25519 JWT/JWS Integration Readiness Check"
echo "======================================================="

PROJECT_ROOT="/home/neurosx/DEVSPACE/whiteflag-rust"
cd "$PROJECT_ROOT"

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]] || [[ ! -d "wf_auth" ]]; then
    echo "❌ Error: Not in whiteflag-rust project root or missing wf_auth module"
    exit 1
fi

echo "✅ Project structure validated"

# Check current dependencies
echo ""
echo "📦 Checking current dependencies..."

echo "Current wf_auth dependencies:"
cat wf_auth/Cargo.toml | grep -A 10 "\[dependencies\]"

echo ""
echo "Current crypto dependencies:"
cat wf_crypto/Cargo.toml | grep -A 20 "\[dependencies\]"

# Check for existing JWT/JWS implementations
echo ""
echo "🔍 Scanning for existing JWT/JWS code..."

JWT_FILES=$(find . -name "*.rs" -exec grep -l "jwt\|jws\|Json.*Web\|token" {} \; 2>/dev/null || true)
if [[ -n "$JWT_FILES" ]]; then
    echo "Found potential JWT-related files:"
    echo "$JWT_FILES"
else
    echo "No existing JWT/JWS implementations found"
fi

# Check for sr25519/signature implementations
echo ""
echo "🔑 Scanning for signature/crypto implementations..."

CRYPTO_FILES=$(find . -name "*.rs" -exec grep -l "sign\|signature\|crypto\|key" {} \; 2>/dev/null || true)
if [[ -n "$CRYPTO_FILES" ]]; then
    echo "Found crypto-related files:"
    echo "$CRYPTO_FILES" | head -10
    if [[ $(echo "$CRYPTO_FILES" | wc -l) -gt 10 ]]; then
        echo "... and $(( $(echo "$CRYPTO_FILES" | wc -l) - 10 )) more"
    fi
else
    echo "No signature/crypto implementations found"
fi

# Check current authentication implementation
echo ""
echo "🔐 Analyzing current authentication implementation..."

if [[ -f "wf_auth/src/lib.rs" ]]; then
    echo "Current authentication methods:"
    grep -n "AuthenticationMethod" wf_auth/src/lib.rs | head -5
    
    echo ""
    echo "Current auth token structure:"
    grep -A 5 -B 5 "struct.*Auth" wf_auth/src/lib.rs || echo "No auth structs found"
else
    echo "❌ wf_auth/src/lib.rs not found"
fi

# Check for Substrate/Polkadot dependencies
echo ""
echo "🌐 Checking for Substrate/Polkadot dependencies..."

SUBSTRATE_DEPS=$(find . -name "*.toml" -exec grep -l "substrate\|polkadot\|schnorrkel\|sr25519" {} \; 2>/dev/null || true)
if [[ -n "$SUBSTRATE_DEPS" ]]; then
    echo "Found Substrate-related dependencies in:"
    echo "$SUBSTRATE_DEPS"
else
    echo "No Substrate/sr25519 dependencies found yet"
fi

# Check compilation status
echo ""
echo "🔨 Testing current compilation..."

if cargo check --quiet 2>/dev/null; then
    echo "✅ Project compiles successfully"
else
    echo "⚠️  Project has compilation issues (may be expected during development)"
fi

# Test current functionality
echo ""
echo "🧪 Testing current authentication functionality..."

if cargo test -p wf_auth --quiet 2>/dev/null; then
    echo "✅ Current authentication tests pass"
else
    echo "⚠️  Authentication tests failing or no tests found"
fi

# Check investigation files
echo ""
echo "📋 Checking investigation/solution files..."

INVESTIGATION_DIR="$PROJECT_ROOT/INVESTIGATION"
if [[ -d "$INVESTIGATION_DIR" ]]; then
    echo "Investigation files present:"
    ls -la "$INVESTIGATION_DIR"
    
    echo ""
    echo "Solution analysis status:"
    if [[ -s "$INVESTIGATION_DIR/solution_analysis.md" ]]; then
        echo "✅ Solution analysis complete ($(wc -l < "$INVESTIGATION_DIR/solution_analysis.md") lines)"
    else
        echo "❌ Solution analysis missing or empty"
    fi
    
    if [[ -s "$INVESTIGATION_DIR/jws_implementation_example.rs" ]]; then
        echo "✅ JWS implementation example present ($(wc -l < "$INVESTIGATION_DIR/jws_implementation_example.rs") lines)"
    else
        echo "❌ JWS implementation example missing or empty"
    fi
    
    if [[ -s "$INVESTIGATION_DIR/updated_wf_auth_cargo.toml" ]]; then
        echo "✅ Updated Cargo.toml ready ($(wc -l < "$INVESTIGATION_DIR/updated_wf_auth_cargo.toml") lines)"
    else
        echo "❌ Updated Cargo.toml missing or empty"
    fi
else
    echo "❌ Investigation directory not found"
fi

# Check system dependencies
echo ""
echo "🛠️  Checking system requirements..."

if command -v rustc &> /dev/null; then
    echo "✅ Rust compiler: $(rustc --version)"
else
    echo "❌ Rust compiler not found"
fi

if command -v cargo &> /dev/null; then
    echo "✅ Cargo: $(cargo --version)"
else
    echo "❌ Cargo not found"
fi

# Integration readiness assessment
echo ""
echo "📊 Integration Readiness Assessment"
echo "=================================="

READINESS_SCORE=0
TOTAL_CHECKS=10

# Check 1: Project structure
if [[ -d "wf_auth" && -d "wf_crypto" ]]; then
    echo "✅ Project structure ready"
    ((READINESS_SCORE++))
else
    echo "❌ Project structure incomplete"
fi

# Check 2: Current auth implementation
if [[ -f "wf_auth/src/lib.rs" ]] && grep -q "AuthenticationMethod" wf_auth/src/lib.rs; then
    echo "✅ Authentication foundation present"
    ((READINESS_SCORE++))
else
    echo "❌ Authentication foundation missing"
fi

# Check 3: Crypto infrastructure
if [[ -f "wf_crypto/src/lib.rs" ]]; then
    echo "✅ Crypto infrastructure present"
    ((READINESS_SCORE++))
else
    echo "❌ Crypto infrastructure missing"
fi

# Check 4: Compilation status
if cargo check --quiet 2>/dev/null; then
    echo "✅ Project compiles"
    ((READINESS_SCORE++))
else
    echo "❌ Compilation issues"
fi

# Check 5: Solution analysis
if [[ -s "$INVESTIGATION_DIR/solution_analysis.md" ]]; then
    echo "✅ Solution analysis complete"
    ((READINESS_SCORE++))
else
    echo "❌ Solution analysis needed"
fi

# Check 6: Implementation example
if [[ -s "$INVESTIGATION_DIR/jws_implementation_example.rs" ]]; then
    echo "✅ Implementation example ready"
    ((READINESS_SCORE++))
else
    echo "❌ Implementation example needed"
fi

# Check 7: Updated dependencies
if [[ -s "$INVESTIGATION_DIR/updated_wf_auth_cargo.toml" ]]; then
    echo "✅ Updated dependencies planned"
    ((READINESS_SCORE++))
else
    echo "❌ Dependency planning needed"
fi

# Check 8: Documentation
if [[ -s "$INVESTIGATION_DIR/implementation_roadmap.md" ]]; then
    echo "✅ Implementation roadmap ready"
    ((READINESS_SCORE++))
else
    echo "❌ Implementation roadmap needed"
fi

# Check 9: System tools
if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    echo "✅ Development tools ready"
    ((READINESS_SCORE++))
else
    echo "❌ Development tools missing"
fi

# Check 10: Issue understanding
if [[ -f "$INVESTIGATION_DIR/issue.md" ]] && grep -q "sr25519" "$INVESTIGATION_DIR/issue.md"; then
    echo "✅ Problem clearly identified"
    ((READINESS_SCORE++))
else
    echo "❌ Problem identification incomplete"
fi

# Final assessment
echo ""
echo "🎯 Final Readiness Score: $READINESS_SCORE/$TOTAL_CHECKS"

if [[ $READINESS_SCORE -ge 8 ]]; then
    echo "🟢 HIGH READINESS - Ready to begin implementation"
    echo ""
    echo "Next steps:"
    echo "1. Update wf_auth/Cargo.toml with new dependencies"
    echo "2. Begin implementing Sr25519JwsToken structure"
    echo "3. Create integration tests"
elif [[ $READINESS_SCORE -ge 5 ]]; then
    echo "🟡 MEDIUM READINESS - Some preparation needed"
    echo ""
    echo "Complete these items first:"
    echo "1. Fix any compilation issues"
    echo "2. Complete missing documentation"
    echo "3. Validate current authentication implementation"
elif [[ $READINESS_SCORE -ge 3 ]]; then
    echo "🟠 LOW READINESS - Significant preparation needed"
    echo ""
    echo "Focus on:"
    echo "1. Stabilize project structure and compilation"
    echo "2. Complete problem analysis and solution design"
    echo "3. Set up proper development environment"
else
    echo "🔴 NOT READY - Major issues to resolve"
    echo ""
    echo "Critical items:"
    echo "1. Fix project structure and basic functionality"
    echo "2. Install required development tools"
    echo "3. Complete initial analysis and planning"
fi

echo ""
echo "For detailed guidance, review:"
echo "- $INVESTIGATION_DIR/solution_analysis.md"
echo "- $INVESTIGATION_DIR/implementation_roadmap.md"
echo "- $INVESTIGATION_DIR/jws_implementation_example.rs"

echo ""
echo "Integration readiness check complete! 🏁"
